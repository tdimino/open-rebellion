
uint __thiscall FUN_00550eb0(void *this,int param_1,int param_2)

{
  uint uVar1;
  uint uVar2;
  undefined4 local_10c [64];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648b46;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_0054fe80(local_10c,(int)this + 0x28);
  local_4 = 0;
  uVar1 = FUN_00550bd0((int)local_10c,param_1 + 4,param_2);
  if (uVar1 != 0) {
    FUN_0054ff00((void *)((int)this + 0x28),(int)local_10c);
  }
  local_4 = 0xffffffff;
  FUN_0054fe60(local_10c);
  uVar2 = 0;
  if (uVar1 != 0) {
    FUN_0054fe80(local_10c,(int)this + 0x128);
    local_4 = 1;
    uVar2 = FUN_00550bd0((int)local_10c,param_1 + 0x74,param_2);
    if (uVar2 != 0) {
      FUN_0054ff00((void *)((int)this + 0x128),(int)local_10c);
    }
    local_4 = 0xffffffff;
    FUN_0054fe60(local_10c);
  }
  ExceptionList = local_c;
  return uVar2;
}

