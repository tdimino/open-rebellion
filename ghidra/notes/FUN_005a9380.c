
void __thiscall FUN_005a9380(void *this,int *param_1)

{
  int iVar1;
  void *this_00;
  
  iVar1 = (**(code **)(*param_1 + 0x20))();
  this_00 = (void *)((int)this + 0x8f8);
  if (iVar1 != 0) {
    this_00 = (void *)((int)this + 0x900);
  }
  FUN_005cea30(this_00,param_1);
  return;
}

