
bool FUN_00562520(void *param_1,void *param_2)

{
  void *this;
  uint *puVar1;
  int iVar2;
  uint uVar3;
  int *piVar4;
  bool bVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064a3b8;
  local_c = ExceptionList;
  uVar3 = 0;
  if (*(int *)((int)param_1 + 0x40) == 0) {
    if (*(int *)((int)param_1 + 0x44) == 0) goto LAB_00562561;
  }
  else if (*(int *)((int)param_1 + 0x44) == 0) {
    uVar3 = 1;
    goto LAB_00562561;
  }
  if (*(int *)((int)param_1 + 0x40) == 0) {
    uVar3 = 2;
  }
LAB_00562561:
  if ((uVar3 == 0) || (3 < uVar3)) {
    bVar5 = false;
  }
  else {
    ExceptionList = &local_c;
    this = (void *)FUN_00506e20();
    bVar5 = this != (void *)0x0;
    piVar4 = (int *)0x0;
    if (this != (void *)0x0) {
      puVar1 = (uint *)FUN_00402e40(param_1,(uint *)&param_1);
      local_4 = 0;
      piVar4 = FUN_004f7d50(this,puVar1,uVar3,param_2,0,0);
      local_4 = 0xffffffff;
      FUN_00619730();
      if ((piVar4 == (int *)0x0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
    }
    if (piVar4 != (int *)0x0) {
      iVar2 = (**(code **)(*piVar4 + 0xbc))(param_2);
      if ((iVar2 == 0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
    }
  }
  ExceptionList = local_c;
  return bVar5;
}

