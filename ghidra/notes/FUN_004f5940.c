
void * __thiscall FUN_004f5940(void *this,uint *param_1)

{
  void *this_00;
  uint uVar1;
  uint uVar2;
  uint *puVar3;
  undefined4 *puVar4;
  uint *puVar5;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063f9e0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  puVar3 = param_1;
  this_00 = *(void **)((int)this + 4);
  local_4 = 0;
  while (this_00 != (void *)0x0) {
    puVar4 = FUN_00403040(this_00,&param_1);
    local_4._0_1_ = 1;
    puVar5 = (uint *)FUN_004f26d0(&local_10,puVar4);
    uVar1 = *puVar3;
    uVar2 = *puVar5;
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if (uVar1 == uVar2) break;
    if (local_10 < (void *)*puVar3) {
      this_00 = *(void **)((int)this_00 + 8);
    }
    else {
      this_00 = *(void **)((int)this_00 + 4);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return this_00;
}

