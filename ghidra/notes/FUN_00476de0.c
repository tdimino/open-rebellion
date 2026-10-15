
void __thiscall FUN_00476de0(void *this,int *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  
  if (param_1 != (int *)0x0) {
    puVar1 = (undefined4 *)FUN_005f54d0((void *)((int)this + 0xd4),param_1[6]);
    if (puVar1 != (undefined4 *)0x0) {
      (**(code **)*puVar1)(1);
      uVar2 = FUN_00479520(param_1);
      *(int *)((int)this + (uVar2 & 0xffff) * 4 + 200) =
           *(int *)((int)this + (uVar2 & 0xffff) * 4 + 200) + -1;
      param_1[0x15] = 0;
      FUN_004ece80(param_1 + 0x16);
      (**(code **)(*param_1 + 0x20))(0);
    }
  }
  return;
}

