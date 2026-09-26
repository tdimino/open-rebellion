
int FUN_00449e00(HDC param_1,int *param_2,int *param_3)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  char *pcVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  int local_a0;
  int local_98;
  undefined4 local_94 [5];
  undefined4 uStack_80;
  int iStack_7c;
  undefined4 uStack_78;
  int iStack_74;
  undefined4 local_68;
  undefined4 local_60 [5];
  undefined4 uStack_4c;
  int iStack_48;
  undefined4 uStack_44;
  int iStack_40;
  undefined4 local_34;
  undefined4 uStack_2c;
  int iStack_28;
  undefined4 uStack_24;
  undefined4 uStack_1c;
  undefined4 uStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f273;
  pvStack_c = ExceptionList;
  local_a0 = 0;
  ExceptionList = &pvStack_c;
  FUN_00601880(local_94,(char *)0x0,0,0,0,0,0,0);
  local_4 = 0;
  FUN_00601880(local_60,(char *)0x0,0,0,0,0,0,0);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00601c60(local_94,10);
  FUN_00601c90(local_94,0xffffff,(HWND)0x0);
  FUN_00403e90(local_94,0x10);
  local_68 = 1;
  FUN_00601c60(local_60,10);
  FUN_00601c90(local_60,0xffffff,(HWND)0x0);
  FUN_00403e90(local_60,0x10);
  local_94[0] = 0x67;
  local_60[0] = 0x67;
  local_34 = 1;
  local_98 = 0;
  iVar6 = 0;
  piVar2 = (int *)(**(code **)(*param_2 + 8))();
  for (piVar3 = (int *)(**(code **)(*param_3 + 8))();
      (piVar2 != (int *)0x0 && (piVar3 != (int *)0x0));
      piVar3 = (int *)(**(code **)(*piVar3 + 0xc))()) {
    FUN_00601b30(local_94,0,local_98);
    pcVar4 = (char *)FUN_00583c40((int)(piVar2 + 5));
    FUN_00601aa0(local_94,pcVar4);
    FUN_00601b80(local_94,param_1);
    iVar1 = iStack_74;
    uStack_24 = uStack_78;
    uStack_2c = uStack_80;
    iStack_28 = iStack_7c;
    FUN_00601b30(local_60,0x67,iVar6);
    pcVar4 = (char *)FUN_00583c40((int)(piVar3 + 5));
    FUN_00601aa0(local_60,pcVar4);
    FUN_00601b80(local_60,param_1);
    uStack_1c = uStack_4c;
    uStack_14 = uStack_44;
    piVar2[10] = 0;
    piVar2[0xb] = local_98;
    uVar7 = iVar1 - iStack_28;
    uVar5 = iStack_40 - iStack_48;
    piVar3[10] = 0x67;
    piVar3[0xb] = iVar6;
    if (uVar5 < uVar7) {
      piVar3[0xb] = (uVar7 - uVar5) + iVar6;
      uVar5 = uVar7;
    }
    local_98 = local_98 + uVar5;
    local_a0 = local_a0 + uVar5;
    iVar6 = iVar6 + uVar5;
    piVar2 = (int *)(**(code **)(*piVar2 + 0xc))();
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00601990((int)local_60);
  local_4 = 0xffffffff;
  FUN_00601990((int)local_94);
  ExceptionList = pvStack_c;
  return local_a0;
}

