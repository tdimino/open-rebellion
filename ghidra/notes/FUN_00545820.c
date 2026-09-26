
bool FUN_00545820(uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  bool bVar2;
  void *this;
  undefined4 *puVar3;
  uint *puVar4;
  uint *puVar5;
  uint *this_00;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puVar4 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006475b0;
  local_c = ExceptionList;
  bVar2 = true;
  if (((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) || (param_2 != 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    this = (void *)FUN_00505d40(puVar4);
    FUN_004ece30(&param_1);
    local_4 = 0;
    puVar4 = (uint *)0x0;
    if (this != (void *)0x0) {
      puVar3 = FUN_0042d170(this,&param_2);
      local_4._0_1_ = 1;
      FUN_004f26d0(&param_1,puVar3);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      puVar4 = FUN_004ece40((uint *)&param_1);
    }
    this_00 = (uint *)0x0;
    puVar5 = FUN_004ece40((uint *)&param_1);
    if (puVar5 != (uint *)0x0) {
      this_00 = (uint *)FUN_00505970((uint *)&param_1);
      puVar4 = this_00;
    }
    bVar2 = puVar4 != (uint *)0x0;
    if (this_00 != (uint *)0x0) {
      bVar2 = FUN_00522280(this_00,param_3);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return bVar2;
}

