
bool __thiscall FUN_00588e80(void *this,void *param_1,uint *param_2,void *param_3)

{
  void *this_00;
  uint *puVar1;
  void *pvVar2;
  uint *puVar3;
  int iVar4;
  int *piVar5;
  bool bVar6;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar1 = param_2;
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fa48;
  local_c = ExceptionList;
  bVar6 = true;
  ExceptionList = &local_c;
  *param_2 = 0;
  local_10 = this;
  param_2 = FUN_004025b0(param_1,(uint *)&local_10);
  local_4 = 0;
  puVar3 = (uint *)FUN_0048a640(this,&param_1);
  param_2 = (uint *)(uint)(*puVar3 != *param_2);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (param_2 != (uint *)0x0) {
    param_2 = (uint *)0x0;
    iVar4 = FUN_0055e550(*(uint *)((int)this_00 + 0x24) >> 6 & 3,
                         (int)*(short *)((int)this_00 + 0x8c),puVar1,(int *)&param_2);
    bVar6 = iVar4 != 0;
    FUN_005897b0();
  }
  if (*puVar1 != 0) {
    piVar5 = (int *)FUN_004ece30(&param_2);
    pvVar2 = param_3;
    local_4 = 1;
    iVar4 = FUN_004ee5c0(this_00,piVar5,param_3);
    if ((iVar4 == 0) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    piVar5 = (int *)FUN_0048a640(this,&param_2);
    local_4 = 2;
    iVar4 = FUN_004ee5c0(this_00,piVar5,pvVar2);
    if ((iVar4 == 0) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return bVar6;
}

