
void __thiscall FUN_0041b5d0(void *this,int *param_1)

{
  uint uVar1;
  
  if (param_1 != (int *)0x0) {
    uVar1 = (**(code **)(*param_1 + 0x28))();
    *(int *)((int)this + (uVar1 & 0xffff) * 4 + 0x35c) =
         *(int *)((int)this + (uVar1 & 0xffff) * 4 + 0x35c) + 1;
  }
  return;
}

