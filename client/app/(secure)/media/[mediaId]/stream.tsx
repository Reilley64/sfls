import { useEffect } from "react";
import { Dimensions, SafeAreaView } from "react-native";

import { useEvent } from "expo";
import { useLocalSearchParams } from "expo-router";
import { VideoView, useVideoPlayer } from "expo-video";

import { useAuthStore } from "~/stores/auth";

export default function Stream() {
  const { bearerToken } = useAuthStore();
  const { mediaId } = useLocalSearchParams();

  const { width, height } = Dimensions.get("window");

  const player = useVideoPlayer(
    {
      uri: `http://192.168.86.123:8080/media/${mediaId}/stream`,
      headers: { Authorization: `Bearer ${bearerToken}` },
    },
    (player) => {
      player.play();
    },
  );

  const { status, error } = useEvent(player, "statusChange", { status: player.status });

  useEffect(() => {
    console.log(status, error);
  }, [status, error]);

  return (
    <SafeAreaView>
      <VideoView player={player} style={{ width, height }} />
    </SafeAreaView>
  );
}
