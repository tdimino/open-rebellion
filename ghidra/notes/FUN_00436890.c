
bool __thiscall FUN_00436890(void *this,uint param_1)

{
  uint uVar1;
  uint uVar2;
  
  uVar2 = *(uint *)(*(int *)((int)this + 0x10) + 0x28);
  uVar1 = *(uint *)(*(int *)((int)this + 0x14) + 0x28);
  if (uVar2 <= uVar1) {
    uVar2 = uVar1;
  }
  if (param_1 < uVar2) {
    param_1 = uVar2 + 1;
  }
  uVar2 = FUN_0041dba0(param_1);
  if (uVar2 != 0) {
    (**(code **)(**(int **)((int)this + 0x10) + 0x174))(uVar2);
    (**(code **)(**(int **)((int)this + 0x14) + 0x174))(uVar2);
  }
  return uVar2 != 0;
}

