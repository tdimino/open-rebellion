
int __thiscall FUN_0053a350(void *this,int *param_1)

{
  int *piVar1;
  ushort uVar2;
  int *this_00;
  int iVar3;
  int iVar4;
  uint local_8;
  undefined4 uStack_4;
  
  piVar1 = param_1;
  iVar4 = 1;
  local_8 = (uint)(*(int *)((int)this + 0x28) != 0);
  FUN_005f4db0(param_1,&local_8);
  if (*(void **)((int)this + 0x28) != (void *)0x0) {
    iVar4 = FUN_005841d0(*(void **)((int)this + 0x28),piVar1);
    uVar2 = FUN_005f50e0(*(int *)((int)this + 0x28));
    param_1 = (int *)(uint)uVar2;
    if (((iVar4 != 0) && (FUN_005f4db0(piVar1,&param_1), iVar4 != 0)) && (param_1 != (int *)0x0)) {
      FUN_005f4d70(piVar1);
    }
    for (this_00 = (int *)FUN_00539f70((int)this); this_00 != (int *)0x0;
        this_00 = (int *)FUN_005c7530((int)this_00)) {
      uStack_4 = (**(code **)(*this_00 + 4))();
      FUN_005f4db0(piVar1,&uStack_4);
      iVar3 = (**(code **)(*this_00 + 0x18))(piVar1);
      iVar4 = 0;
      if (iVar3 != 0) {
        iVar4 = FUN_0053a350(this_00,piVar1);
      }
    }
  }
  return iVar4;
}

