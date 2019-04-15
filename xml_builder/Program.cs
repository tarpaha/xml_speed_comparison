using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Xml.Serialization;

namespace xml_builder
{
    public static class Program
    {
        private const int BundlesCount = 25000;
        private const int AssetsPerBundleMin = 1;
        private const int AssetsPerBundleMax = 10;
        private const int BundleSizeMin = 1000;
        private const int BundleSizeMax = 10000;
        private const string OutputFilename = "resource_map.xml";

        private static void Main()
        {
            var rand = new Random(DateTime.Now.Millisecond);

            var assetId = 1;
            var bundles = new List<Bundle>();
            for (var bundleId = 1; bundleId <= BundlesCount; bundleId++)
            {
                var assetsInBundleCount = rand.Next(AssetsPerBundleMin, AssetsPerBundleMax);
                var assets = Enumerable
                    .Range(0, assetsInBundleCount)
                    .Select(id => id + assetId)
                    .Select(id => new Asset {AssetPath = "asset" + id.ToString("D6") })
                    .ToArray();
                bundles.Add(new Bundle
                {
                    Filename = "bundle" + bundleId.ToString("D6"),
                    DownloadSize = rand.Next(BundleSizeMin, BundleSizeMax),
                    Assets = assets
                });
                assetId += assetsInBundleCount;
            }
            var resourceMap = new ResourceMap {Bundles = bundles.ToArray()};
            
            var formatter = new XmlSerializer(typeof(ResourceMap));
            using (var fs = new FileStream(OutputFilename, FileMode.Create))
            {
                formatter.Serialize(fs, resourceMap);
            }
            
            Console.WriteLine($"Data saved to {OutputFilename}");
        }
    }
}
