
void __thiscall FUN_0048a920(void *this,int *param_1)

{
  int iVar1;
  
  if (param_1 != (int *)0x0) {
    iVar1 = (**(code **)(*param_1 + 0xc))();
    if ((iVar1 == 1) && (*(uint *)((int)this + 0x2c) < (uint)param_1[0xb])) {
      *(int *)((int)this + 0x2c) = param_1[0xb];
    }
    FUN_00536fe0((void *)((int)this + 4),param_1);
  }
  return;
}

