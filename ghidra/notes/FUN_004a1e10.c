
void __thiscall FUN_004a1e10(void *this,uint *param_1)

{
  bool bVar1;
  uint *puVar2;
  int *this_00;
  uint *puVar3;
  int iVar4;
  uint uVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006377e8;
  local_c = ExceptionList;
  if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar1) {
    this_00 = FUN_004f3110(*(int *)(*(int *)((int)this + 0x14c) + 0x9c),puVar2);
    if (this_00 != (int *)0x0) {
      if ((*(byte *)(this_00 + 0x1e) & 1) == 0) {
        uVar5 = 0x17;
      }
      else {
        uVar5 = 0x18;
      }
      FUN_004a1ba0(this,uVar5,1);
      puVar3 = FUN_004025b0(this_00,(uint *)&param_1);
      local_4 = 0;
      iVar4 = FUN_006098f0(*(void **)((int)this + 0x1c0),*puVar3 & 0xffffff);
      local_4 = 0xffffffff;
      FUN_00619730();
      if (iVar4 != 0) {
        *(uint *)(iVar4 + 0x3c) = *(uint *)(iVar4 + 0x3c) | 1;
        FUN_0060a280(*(void **)((int)this + 0x1c0));
      }
    }
  }
  if ((*puVar2 >> 0x18 < 0x50) || (0x7f < *puVar2 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    iVar4 = FUN_0060a860(*(void **)((int)this + 0x1b4),*puVar2 & 0xffffff);
    if (iVar4 != 0) {
      FUN_004a0c60(this,iVar4,1);
    }
  }
  ExceptionList = local_c;
  return;
}

