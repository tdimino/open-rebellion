
int __thiscall FUN_0051af00(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  uint *puVar3;
  void *this_00;
  int *piVar4;
  int iVar5;
  bool bVar6;
  int local_28 [6];
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642ecf;
  local_c = ExceptionList;
  local_28[5] = 0;
  local_28[0] = 1;
  ExceptionList = &local_c;
  local_10 = this;
  iVar2 = FUN_0055d500(local_28);
  bVar6 = local_28[0] == 0;
  if (bVar6) {
    puVar3 = (uint *)((int)this + 0x154);
  }
  else {
    local_28[1] = 0x92000121;
    local_28[5] = 1;
    puVar3 = (uint *)(local_28 + 1);
    local_4 = 0;
  }
  this_00 = (void *)FUN_00505f60(puVar3);
  local_4 = 0xffffffff;
  if (!bVar6) {
    FUN_00619730();
  }
  pvVar1 = param_1;
  if ((this_00 == (void *)0x0) || (iVar2 == 0)) {
    bVar6 = false;
  }
  else {
    bVar6 = true;
  }
  piVar4 = (int *)0x0;
  if (this_00 != (void *)0x0) {
    param_1 = (void *)0x8000004;
    local_4 = 1;
    piVar4 = FUN_004f7d50(this_00,(uint *)&param_1,*(uint *)((int)this_00 + 0x24) >> 6 & 3,pvVar1,1,
                          1);
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((piVar4 == (int *)0x0) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
  }
  iVar5 = 0;
  local_28[4] = 0;
  local_28[3] = 0;
  local_28[2] = 0;
  iVar2 = FUN_0055d5b0(local_28 + 4,local_28 + 3,local_28 + 2);
  if ((iVar2 != 0) && (bVar6)) {
    iVar5 = 1;
  }
  if (piVar4 != (int *)0x0) {
    iVar2 = FUN_0051be90(piVar4,local_28[4],local_28[3],local_28[2],pvVar1);
    if ((iVar2 == 0) || (iVar5 == 0)) {
      iVar5 = 0;
    }
    else {
      iVar5 = 1;
    }
  }
  ExceptionList = local_c;
  return iVar5;
}

