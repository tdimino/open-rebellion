
bool __fastcall FUN_004c0ba0(int param_1)

{
  LPBYTE pBVar1;
  void *pvVar2;
  int iVar3;
  char *pcVar4;
  int local_cc;
  int aiStack_c8 [2];
  undefined1 local_c0 [8];
  undefined4 local_b8 [20];
  int local_68 [3];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a3a3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00616620(local_68,1);
  local_4 = 0;
  pBVar1 = FUN_00401560();
  FUN_00615db0(&local_cc,1);
  local_4 = CONCAT31(local_4._1_3_,1);
  if (pBVar1 != (LPBYTE)0x0) {
    pcVar4 = &DAT_006a7c4c;
    pvVar2 = FUN_00616110(local_c0,(char *)pBVar1);
    FUN_00616110(pvVar2,pcVar4);
    FUN_00618b60(pBVar1);
  }
  pcVar4 = FUN_005f49e0(0x618);
  FUN_00616110(local_c0,pcVar4);
  pcVar4 = FUN_005f49e0(0x61c);
  pvVar2 = FUN_00616110(local_c0,pcVar4);
  FUN_00615f00(pvVar2,0);
  pcVar4 = (char *)FUN_00615680(*(int *)((int)aiStack_c8 + *(int *)(local_cc + 4)));
  FUN_005f4a60(local_68,pcVar4);
  FUN_00615660(*(void **)((int)aiStack_c8 + *(int *)(local_cc + 4)),0);
  iVar3 = FUN_005f49b0();
  if (iVar3 == 0) {
    FUN_0049c6f0((void *)(param_1 + 0x138),local_68);
    FUN_006168a0(local_68);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00615ea0((int)local_b8);
  FUN_00615440(local_b8);
  local_4 = 0xffffffff;
  FUN_00616810((int)auStack_5c);
  FUN_00615440(auStack_5c);
  ExceptionList = pvStack_c;
  return iVar3 == 0;
}

