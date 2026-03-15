
void __thiscall FUN_004e4e20(void *this,void *param_1)

{
  int *piVar1;
  int local_8;
  int local_4;
  
  FUN_005f5b20(this);
  FUN_005f5610((void *)((int)this + 0xc),param_1);
  FUN_005f4d90(param_1,&local_8);
  FUN_005f4d90(param_1,(int)this + 0x28);
  for (; local_8 != 0; local_8 = local_8 + -1) {
    FUN_005f4d90(param_1,&local_4);
    piVar1 = (int *)FUN_0051f730(local_4);
    if (piVar1 != (int *)0x0) {
      (**(code **)(*piVar1 + 4))(param_1);
      FUN_004e4f20(this,piVar1);
    }
  }
  return;
}

