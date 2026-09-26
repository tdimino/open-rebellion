
void * __thiscall FUN_004727e0(void *this,int param_1)

{
  undefined4 uVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632843;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590(this);
  local_4 = 0;
  FUN_004ad770((undefined4 *)((int)this + 0x30));
  *(undefined2 *)((int)this + 0x48) = 0;
  *(undefined2 *)((int)this + 0x4a) = 0;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_0065a358;
  if (param_1 < 1) {
    param_1 = 1;
  }
  uVar1 = FUN_00618b70(param_1 * 4);
  *(undefined4 *)((int)this + 0x20) = uVar1;
  iVar2 = 0;
  if (0 < param_1) {
    do {
      iVar2 = iVar2 + 1;
      *(undefined4 *)(*(int *)((int)this + 0x20) + -4 + iVar2 * 4) = 0;
    } while (iVar2 < param_1);
  }
  *(int *)((int)this + 0x24) = param_1;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined4 *)((int)this + 0x40) = 0;
  ExceptionList = local_c;
  return this;
}

