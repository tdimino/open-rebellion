
int __cdecl FUN_005ff020(char *param_1,char *param_2,int param_3)

{
  int iVar1;
  int iVar2;
  undefined1 uVar3;
  int iVar4;
  void *pvVar5;
  undefined4 *puVar6;
  LPCSTR pCVar7;
  char *pcVar8;
  int local_70;
  int aiStack_6c [2];
  undefined1 local_64 [8];
  undefined4 local_5c [8];
  int iStack_3c;
  _RTL_CRITICAL_SECTION _Stack_38;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655f5e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar4 = FUN_005fefd0(param_3);
  if (iVar4 == 0) {
    FUN_00615db0(&local_70,1);
    local_4 = 0;
    if (param_1 != (char *)0x0) {
      pcVar8 = &DAT_006a7c4c;
      pvVar5 = FUN_00616110(local_64,param_1);
      FUN_00616110(pvVar5,pcVar8);
    }
    pvVar5 = FUN_00616110(local_64,param_2);
    FUN_00615f00(pvVar5,0);
    pvVar5 = (void *)FUN_00618b70(0x14);
    local_4._0_1_ = 1;
    if (pvVar5 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = FUN_0060ed60(pvVar5,param_2);
    }
    local_4._0_1_ = 0;
    uVar3 = (undefined1)local_4;
    local_4._0_1_ = 0;
    if (puVar6[4] == 0) {
      if (puVar6 != (undefined4 *)0x0) {
        (**(code **)*puVar6)(1);
      }
      pvVar5 = (void *)FUN_00618b70(0x14);
      local_4._0_1_ = 2;
      if (pvVar5 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        pCVar7 = (LPCSTR)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(local_70 + 4)));
        puVar6 = FUN_0060ed60(pvVar5,pCVar7);
      }
      local_4._0_1_ = 0;
      FUN_00615660(*(void **)((int)aiStack_6c + *(int *)(local_70 + 4)),0);
      iVar1 = *(int *)(local_70 + 4);
      if (*(int *)((int)&iStack_3c + iVar1) < 0) {
        FUN_00615ee0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar1));
      }
      iVar2 = *(int *)((int)&iStack_3c + iVar1);
      *(undefined4 *)((int)aiStack_6c + iVar1 + 4) = 0;
      uVar3 = (undefined1)local_4;
      if (iVar2 < 0) {
        FUN_00615ef0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar1));
        uVar3 = (undefined1)local_4;
      }
    }
    local_4._0_1_ = uVar3;
    if (puVar6 != (undefined4 *)0x0) {
      puVar6[3] = param_3;
      FUN_005f58d0(&DAT_006be4c8,(int)puVar6);
      iVar4 = puVar6[4];
    }
    local_4 = 0xffffffff;
    FUN_00615ea0((int)local_5c);
    FUN_00615440(local_5c);
  }
  ExceptionList = local_c;
  return iVar4;
}

