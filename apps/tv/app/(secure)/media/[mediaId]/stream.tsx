import { useEffect } from "react";
import { Dimensions, SafeAreaView } from "react-native";

import { useLocalSearchParams } from "expo-router";
import { VideoView, useVideoPlayer } from "expo-video";

import { useMutation } from "@tanstack/react-query";
import { cssInterop } from "nativewind";

import { useAuthStore } from "~/stores/auth";

import { useTRPC } from "~/lib/trpc";

cssInterop(VideoView, {
  className: {
    target: "style",
  },
});

export default function Stream() {
  const auth = useAuthStore();
  const { mediaId } = useLocalSearchParams();
  const trpc = useTRPC();

  const { width, height } = Dimensions.get("window");

  const player = useVideoPlayer(
    {
      uri: `http://192.168.86.215:10000/media/${mediaId}/stream`,
      headers: { Authorization: `Bearer ${auth.bearerToken}` },
    },
    (player) => {
      player.play();
    },
  );

  const heartbeatMutation = useMutation(trpc.media._media_id.stream.heartbeat.post.mutationOptions());

  useEffect(() => {
    const interval = setInterval(() => {
      heartbeatMutation.mutate({
        params: { mediaId: mediaId as string },
        headers: { Authorization: `Bearer ${auth.bearerToken}` },
        body: { position: player.currentTime },
      });
    }, 900000);

    return () => clearInterval(interval);
  }, [heartbeatMutation]);

  return (
    <SafeAreaView className="bg-backgroud">
      <VideoView allowsFullscreen={false} className="bg-black" player={player} style={{ width, height }} />
    </SafeAreaView>
  );
}
