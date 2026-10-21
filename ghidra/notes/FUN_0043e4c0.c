
void __thiscall FUN_0043e4c0(void *this,int *param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  if (param_1 != (int *)0x0) {
    iVar1 = *param_1;
    uVar2 = (**(code **)(*(int *)this + 0xc))((int)this + 0x20,*(undefined4 *)((int)this + 0x28));
    (**(code **)(iVar1 + 0x20))(uVar2);
  }
  return;
}

