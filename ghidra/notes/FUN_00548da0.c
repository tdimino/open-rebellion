
bool FUN_00548da0(uint *param_1,void *param_2)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  uint *puVar4;
  undefined3 extraout_var;
  bool bVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647b68;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = (int *)FUN_00505d40(param_1);
  bVar5 = piVar2 != (int *)0x0;
  if ((piVar2 != (int *)0x0) &&
     (((piVar2[0x14] & 0x1000U) != 0 || ((*(byte *)(piVar2 + 0x1e) & 0x40) != 0)))) {
    FUN_004ece30(&param_1);
    local_4 = 0;
    iVar3 = (**(code **)(*piVar2 + 0xc))(&param_1);
    if ((iVar3 == 0) || (!bVar5)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    bVar5 = false;
    if (bVar1) {
      iVar3 = FUN_004ece60((uint *)&param_1);
      bVar5 = iVar3 != 0;
    }
    puVar4 = FUN_004ece40((uint *)&param_1);
    if (puVar4 != (uint *)0x0) {
      bVar1 = FUN_00548e90((uint *)&param_1,param_2);
      if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return bVar5;
}

