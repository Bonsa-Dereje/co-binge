import express from "express";
import { MongoClient } from "mongodb";
import dotenv from "dotenv";

dotenv.config();

const app = express();
app.use(express.json());

const client = new MongoClient(process.env.MONGO_URI);

let collection;

/**
 * Fetch time from API and add +5 seconds
 */
async function fetchHostTime() {
    const res = await fetch("https://time.now/developer/api/ip");
    const data = await res.json();

    const date = new Date(data.datetime);

    // +5 seconds adjustment
    const adjusted = new Date(date.getTime() + 5000);

    const syncTime = `${String(adjusted.getHours()).padStart(2, "0")}:` +
                      `${String(adjusted.getMinutes()).padStart(2, "0")}:` +
                      `${String(adjusted.getSeconds()).padStart(2, "0")}`;

    return syncTime;
}

/**
 * Save ONLY syncTime into MongoDB
 */
async function saveTime() {
    const syncTime = await fetchHostTime();

    await collection.updateOne(
        { _id: "hostTime" },
        {
            $set: {
                syncTime
            }
        },
        { upsert: true }
    );

    console.log(`✅ Saved syncTime: ${syncTime}`);
}

/**
 * Connect DB + initial save
 */
async function connectDB() {
    try {
        await client.connect();

        const db = client.db("timeSync");
        collection = db.collection("timeSync");

        console.log("✅ Connected to MongoDB");

        // Save immediately on startup
        await saveTime();

    } catch (err) {
        console.error("❌ MongoDB error:", err);
    }
}

connectDB();

/**
 * Manual trigger endpoint
 */
app.post("/save-time", async (req, res) => {
    try {
        await saveTime();

        res.json({
            success: true,
            message: "syncTime saved",
        });
    } catch (err) {
        console.error(err);
        res.status(500).json({ error: "Failed to save syncTime" });
    }
});

/**
 * Check DB contents
 */
app.get("/check", async (req, res) => {
    const doc = await collection.findOne({ _id: "hostTime" });
    res.json(doc || { message: "nothing found" });
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`🚀 Server running on port ${PORT}`);
});