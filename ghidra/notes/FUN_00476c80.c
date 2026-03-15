
void __thiscall FUN_00476c80(void *this,int *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  
  if (param_1 != (int *)0x0) {
    puVar1 = (undefined4 *)FUN_005f54d0((void *)((int)this + 0xb0),param_1[6]);
    if (puVar1 != (undefined4 *)0x0) {
      (**(code **)*puVar1)(1);
      uVar2 = FUN_004794b0((int)param_1);
      *(int *)((int)this + (uVar2 & 0xffff) * 4 + 0xa0) =
           *(int *)((int)this + (uVar2 & 0xffff) * 4 + 0xa0) + -1;
      (**(code **)(*param_1 + 0x1c))(0);
      param_1[0x1c] = 0;
      FUN_004ece80(param_1 + 0x1d);
      (**(code **)(*param_1 + 0x1c))(0);
    }
  }
  return;
}

