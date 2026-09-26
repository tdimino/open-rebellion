
uint __thiscall FUN_00509890(void *this,uint *param_1)

{
  int iVar1;
  uint uVar2;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641a18;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 3;
  uVar2 = 0;
  if ((*(uint *)((int)this + 0x88) >> 5 & 1) != 0) {
    FUN_004ffe70(local_2c,this,1);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    iVar1 = FUN_0052bed0((int)local_2c);
    uVar2 = (uint)(iVar1 != 0);
    if (uVar2 != 0) {
      iVar1 = FUN_0052bed0((int)local_2c);
      *param_1 = *(uint *)(iVar1 + 0x24) >> 6 & 3;
    }
    local_4 = 0xffffffff;
    FUN_004fff70(local_2c);
  }
  ExceptionList = local_c;
  return uVar2;
}

