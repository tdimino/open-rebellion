
void __cdecl FUN_00417670(int param_1)

{
  DAT_006b14a8 = param_1;
  FUN_005f4250((HKEY)s_HKEY_LOCAL_MACHINE_006a844c,s_SOFTWARE_LucasArts_Entertainment_006a81fc,
               s_MusicVolume_006a8584);
  if (((DAT_006b1490 & 1) == 0) && (DAT_006b147c != 0)) {
    FUN_00610c80(0x6b1478);
    if ((DAT_006b1488 >> 5 & 1) != 0) {
      FUN_00610cb0(0x6b1478,DAT_006b14a8);
    }
  }
  return;
}

