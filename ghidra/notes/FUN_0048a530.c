
void __thiscall FUN_0048a530(void *this,uint param_1)

{
  int iVar1;
  
  if ((*(uint *)((int)this + 0x50) & param_1) == 0) {
    if (param_1 != 0) goto LAB_0048a55b;
    *(undefined4 *)((int)this + 0x50) = 0;
  }
  else {
    *(uint *)((int)this + 0x50) = ~param_1 & *(uint *)((int)this + 0x50);
  }
  FUN_0041db40(param_1);
LAB_0048a55b:
  for (iVar1 = thunk_FUN_005f5060((int)this + 0x10); iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10)) {
    if (((*(uint *)(iVar1 + 0x34) & param_1) != 0) || (param_1 == 0)) {
      *(uint *)(iVar1 + 0x24) = *(uint *)(iVar1 + 0x24) & 0xffffffef;
    }
  }
  return;
}

