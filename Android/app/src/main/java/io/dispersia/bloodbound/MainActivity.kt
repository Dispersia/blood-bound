package io.dispersia.bloodbound

import android.os.Bundle
import android.os.PersistableBundle
import android.view.WindowInsets
import android.view.WindowInsetsController
import androidx.core.view.WindowCompat
import com.google.androidgamesdk.GameActivity

class MainActivity : GameActivity() {
  companion object {
    init {
      System.loadLibrary("blood_bound")
    }
  }

  override fun onCreate(savedInstanceState: Bundle?, persistentState: PersistableBundle?) {
    super.onCreate(savedInstanceState, persistentState)

    WindowCompat.setDecorFitsSystemWindows(window, false)

    window.insetsController?.let { controller ->
      controller.hide(WindowInsets.Type.statusBars() or WindowInsets.Type.navigationBars())
      controller.systemBarsBehavior =
        WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    }
  }
}
