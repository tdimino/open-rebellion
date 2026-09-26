
bool FUN_00556620(int *param_1,uint *param_2)

{
  int *piVar1;
  uint *puVar2;
  int iVar3;
  uint uVar4;
  int *unaff_EBX;
  bool bVar5;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649680;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_2 = 1;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_004ece30(&param_2);
  piVar1 = param_1;
  local_4 = CONCAT31(local_4._1_3_,1);
  iVar3 = (**(code **)(*param_1 + 0xc))(&local_10);
  bVar5 = iVar3 != 0;
  FUN_004f26d0(&param_1,piVar1 + 0xf);
  if (param_1 != unaff_EBX) {
    uVar4 = FUN_00556430(piVar1,(uint *)&param_1,(uint *)&stack0xffffffec,param_2);
    if ((uVar4 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
  }
  puStack_8 = (undefined1 *)((uint)puStack_8 & 0xffffff00);
  *puVar2 = (uint)~piVar1[0x14] >> 0xb & 1;
  FUN_00619730();
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_10;
  return bVar5;
}

