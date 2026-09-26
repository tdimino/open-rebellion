
void __thiscall FUN_004bf1f0(void *this,void *param_1)

{
  FUN_004e5070(this,param_1);
  FUN_005f4d90(param_1,(int)this + 0x38);
  FUN_005f4d90(param_1,(int)this + 0x3c);
  FUN_004ece90((void *)((int)this + 0x50),param_1);
  FUN_005f4d90(param_1,(int)this + 0x54);
  FUN_005f4d90(param_1,(int)this + 0x58);
  FUN_005f4d90(param_1,(int *)((int)this + 0x5c));
  FUN_004e5160((void *)((int)this + 100),param_1);
  if (*(int *)((int)this + 0x5c) != 0) {
    FUN_004bf7c0((int)this);
    if (*(int **)((int)this + 0x60) != (int *)0x0) {
      (**(code **)(**(int **)((int)this + 0x60) + 4))(param_1);
      *(void **)(*(int *)((int)this + 0x60) + 0x2c) = (void *)((int)this + 100);
    }
  }
  return;
}

