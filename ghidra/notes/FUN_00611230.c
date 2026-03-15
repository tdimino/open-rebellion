
void FUN_00611230(void)

{
  undefined4 *puVar1;
  LONG LVar2;
  undefined4 *puVar3;
  int iVar4;
  
  LVar2 = InterlockedExchange((LONG *)&Target_006be660,1);
  if (LVar2 == 0) {
    puVar3 = (undefined4 *)thunk_FUN_005f5060(0x6be650);
    while (puVar1 = puVar3, puVar1 != (undefined4 *)0x0) {
      puVar3 = (undefined4 *)puVar1[4];
      if (puVar1[7] == 1) {
        FUN_00612100(&DAT_006be650,puVar1);
      }
    }
    puVar3 = (undefined4 *)thunk_FUN_005f5060(0x6be5d0);
    while (puVar1 = puVar3, puVar1 != (undefined4 *)0x0) {
      puVar3 = (undefined4 *)puVar1[4];
      if ((puVar1[0x15] == 0) && (iVar4 = FUN_006128e0((uint)puVar1), iVar4 == 0)) {
        FUN_00612100(&DAT_006be5d0,puVar1);
      }
    }
    InterlockedExchange((LONG *)&Target_006be660,0);
  }
  return;
}

