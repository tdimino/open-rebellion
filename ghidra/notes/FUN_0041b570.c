
void __thiscall FUN_0041b570(void *this,int *param_1)

{
  uint uVar1;
  
  if (param_1 != (int *)0x0) {
    uVar1 = (**(code **)(*param_1 + 0x40))();
    *(int *)((int)this + (uVar1 & 0xffff) * 4 + 800) =
         *(int *)((int)this + (uVar1 & 0xffff) * 4 + 800) + 1;
  }
  return;
}

