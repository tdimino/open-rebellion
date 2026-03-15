
void __thiscall FUN_005c5a50(void *this,uint param_1,undefined4 param_2)

{
  int *piVar1;
  uint uVar2;
  
  if (param_1 == 0x15) {
    *(undefined4 *)((int)this + 0xa4) = 0;
    piVar1 = (int *)FUN_00604500((void *)((int)this + 0x6c),0x15);
    if (piVar1 != (int *)0x0) {
      (**(code **)(*piVar1 + 0x3c))(param_2);
    }
    FUN_005c5760(this,0x15);
  }
  else if (*(int *)((int)this + 0xa4) == 0) {
    uVar2 = param_1;
    if (param_1 == 0) {
      uVar2 = *(uint *)((int)this + 0xa0);
    }
    piVar1 = (int *)FUN_00604500((void *)((int)this + 0x6c),uVar2);
    if (piVar1 != (int *)0x0) {
      (**(code **)(*piVar1 + 0x3c))(param_2);
    }
    FUN_005c5760(this,param_1);
    return;
  }
  return;
}

