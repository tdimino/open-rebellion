
undefined4 FUN_00442430(void)

{
  bool bVar1;
  undefined4 *puVar2;
  HWND pHVar3;
  HWND pHVar4;
  bool bVar5;
  
  puVar2 = DAT_006b28f4;
  bVar1 = false;
  bVar5 = DAT_006b28f4 != (undefined4 *)0x0;
  DAT_006b28f4 = (undefined4 *)0x0;
  if (bVar5) {
    if ((DAT_006b28fc != (HWND)0x0) &&
       ((pHVar4 = (HWND)puVar2[6], pHVar3 = GetFocus(), pHVar3 == pHVar4 ||
        (pHVar4 = GetFocus(), pHVar4 == DAT_006b2900)))) {
      bVar1 = true;
    }
    GetFocus();
    pHVar4 = *(HWND *)(puVar2[8] + 0x18);
    FUN_00442c60();
    DestroyWindow((HWND)puVar2[6]);
    (**(code **)*puVar2)(1);
    if (bVar1) {
      SetFocus(DAT_006b28fc);
    }
    DAT_006b28fc = (HWND)0x0;
    UpdateWindow(pHVar4);
    return 1;
  }
  DAT_006b28f4 = (undefined4 *)0x0;
  return 1;
}

