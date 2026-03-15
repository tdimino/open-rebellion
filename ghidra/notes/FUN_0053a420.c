
int __thiscall FUN_0053a420(void *this,void *param_1)

{
  int *this_00;
  int iVar1;
  int iVar2;
  
  iVar2 = 1;
  if (*(int *)((int)this + 0x28) != 0) {
    FUN_005f60f0(param_1);
    for (this_00 = (int *)FUN_00539f70((int)this); this_00 != (int *)0x0;
        this_00 = (int *)FUN_005c7530((int)this_00)) {
      iVar1 = (**(code **)(*this_00 + 8))();
      if (iVar1 != 0) {
        (**(code **)(*this_00 + 4))();
        FUN_005f60f0(param_1);
        iVar2 = (**(code **)(*this_00 + 0x1c))(param_1);
        if (iVar2 != 0) {
          iVar2 = FUN_0053a420(this_00,param_1);
        }
      }
    }
    FUN_005f60f0(param_1);
  }
  return iVar2;
}

