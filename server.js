import express from "express";
import { MongoClient } from "mongodb";
import dotenv from "dotenv";

dotenv.config();

const app = express();
app.use(express.json());

const client = new MongoClient(process.env.MONGO_URI);

let collection;

/**
 * Fetch time from API (same logic as your Svelte app)
 */
async function fetchHostTime() {
    const res = await fetch("https://time.now/developer/api/ip");
    const data = await res.json();

    const date = new Date(data.datetime);

    const plusFive = new Date(date.getTime() + 5000);

    return {
        utcTime: date.toISOString(),
        plusFiveTime: plusFive.toISOString(),
        hh: String(date.getHours()).padStart(2, "0"),
        mm: String(date.getMinutes()).padStart(2, "0"),
        ss: String(date.getSeconds()).padStart(2, "0"),
        hh2: String(plusFive.getHours()).padStart(2, "0"),
        mm2: String(plusFive.getMinutes()).padStart(2, "0"),
        ss2: String(plusFive.getSeconds()).padStart(2, "0")
    };
}

/**
 * Save time into MongoDB
 */
async function saveTime(deviceId = "server-device") {
    const timeData = await fetchHostTime();

    await collection.updateOne(
        { type: "hostTime" },
        {
            $set: {
                deviceId,
                utcTime: timeData.utcTime,
                plusFiveTime: timeData.plusFiveTime,
                hh: timeData.hh,
                mm: timeData.mm,
                ss: timeData.ss,
                hh2: timeData.hh2,
                mm2: timeData.mm2,
                ss2: timeData.ss2,
                savedAt: new Date()
            }
        },
        { upsert: true }
    );

    console.log("✅ Time saved to MongoDB:", timeData.hh + ":" + timeData.mm + ":" + timeData.ss);
}

/**
 * Connect DB + run initial save
 */
async function connectDB() {
    try {
        await client.connect();

        const db = client.db("timeSync");
        collection = db.collection("timeSync");

        console.log("✅ Connected to MongoDB");

        // 🚀 Run immediately when server starts
        await saveTime("server-startup");

    } catch (err) {
        console.error("❌ MongoDB error:", err);
    }
}

connectDB();

/**
 * Optional endpoint (same logic as frontend "Host" button)
 */
app.post("/save-time", async (req, res) => {
    try {
        const { deviceId } = req.body;

        await saveTime(deviceId);

        res.json({ success: true, message: "Time saved" });
    } catch (err) {
        console.error(err);
        res.status(500).json({ error: "Failed to save time" });
    }
});

/**
 * Check DB contents
 */
app.get("/check", async (req, res) => {
    const doc = await collection.findOne({ type: "hostTime" });
    res.json(doc || { message: "nothing found" });
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`🚀 Server running on port ${PORT}`);
});