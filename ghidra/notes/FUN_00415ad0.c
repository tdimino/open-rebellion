
void * __thiscall FUN_00415ad0(void *this,void *param_1,int param_2,int param_3)

{
  int iVar1;
  int iVar2;
  LPBYTE pBVar3;
  void *pvVar4;
  uint uVar5;
  char *pcVar6;
  undefined4 local_d8 [3];
  int local_cc;
  int aiStack_c8 [2];
  undefined1 local_c0 [8];
  undefined4 local_b8 [8];
  int iStack_98;
  _RTL_CRITICAL_SECTION _Stack_94;
  undefined1 local_68 [12];
  undefined4 auStack_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ac38;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_d8);
  local_4 = 1;
  FUN_00616620(local_68,1);
  local_4._0_1_ = 2;
  FUN_00615db0(&local_cc,1);
  local_4._0_1_ = 3;
  if (param_2 != 0) {
    pBVar3 = FUN_00401760();
    if (pBVar3 != (LPBYTE)0x0) {
      pcVar6 = &DAT_006a7c4c;
      pvVar4 = FUN_00616110(local_c0,(char *)pBVar3);
      FUN_00616110(pvVar4,pcVar6);
      FUN_00618b60(pBVar3);
    }
    pcVar6 = FUN_005f49e0(0x610);
    FUN_00616110(local_c0,pcVar6);
    if ((param_3 == 0) &&
       (((*(int *)((int)this + 0x104) != 7 || (*(int *)((int)this + 0xe4) != 2)) &&
        (DAT_006b1208 == 0)))) {
      uVar5 = 0x611;
    }
    else {
      uVar5 = 0x612;
    }
    pcVar6 = FUN_005f49e0(uVar5);
    pvVar4 = FUN_00616110(local_c0,pcVar6);
    pvVar4 = FUN_00616e10(pvVar4,param_2);
    FUN_00615f00(pvVar4,0);
    pcVar6 = (char *)FUN_00615680(*(int *)((int)aiStack_c8 + *(int *)(local_cc + 4)));
    FUN_005f35e0(local_d8,pcVar6);
    FUN_00615660(*(void **)((int)aiStack_c8 + *(int *)(local_cc + 4)),0);
    iVar1 = *(int *)(local_cc + 4);
    if (*(int *)((int)&iStack_98 + iVar1) < 0) {
      FUN_00615ee0((LPCRITICAL_SECTION)((int)&_Stack_94.DebugInfo + iVar1));
    }
    iVar2 = *(int *)((int)&iStack_98 + iVar1);
    *(undefined4 *)((int)aiStack_c8 + iVar1 + 4) = 0;
    if (iVar2 < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)&_Stack_94.DebugInfo + iVar1));
    }
  }
  FUN_005f2f90(param_1,(int)local_d8);
  local_4._0_1_ = 2;
  FUN_00615ea0((int)local_b8);
  FUN_00615440(local_b8);
  local_4._0_1_ = 1;
  FUN_00616810((int)auStack_5c);
  FUN_00615440(auStack_5c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_d8);
  ExceptionList = pvStack_c;
  return param_1;
}

