
undefined4 __fastcall FUN_0052e640(int param_1)

{
  void *this;
  int *piVar1;
  undefined4 uVar2;
  uint local_24;
  undefined4 local_20;
  int local_1c [2];
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645228;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00520570(local_1c);
  local_4 = 0;
  this = (void *)FUN_00506630();
  while (this != (void *)0x0) {
    if ((((*(byte *)((int)this + 0x50) & 1) != 0) && ((*(byte *)((int)this + 0x60) & 1) == 0)) &&
       (((*(uint *)((int)this + 0x24) ^ *(uint *)(param_1 + 0x24)) & 0xc0) == 0)) {
      piVar1 = FUN_0052e740(this,local_14);
      local_4._0_1_ = 2;
      FUN_0051c9c0(local_1c,piVar1);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    local_24 = 0x2c;
    local_20 = 0x2d;
    local_4._0_1_ = 1;
    this = (void *)FUN_004f6010(this,&local_24,1);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  uVar2 = FUN_0051ca30((void *)(param_1 + 0x60),local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar2;
}

