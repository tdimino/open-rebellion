
void __thiscall FUN_004ef640(int *param_1,undefined4 param_2)

{
  int iVar1;
  int *piVar2;
  int iStack_4;
  
  piVar2 = &iStack_4;
  iStack_4 = 0;
  iVar1 = (**(code **)(*param_1 + 0x1f0))();
  iVar1 = FUN_0053ea00(iVar1,piVar2);
  if (iVar1 != 0) {
    (**(code **)(*param_1 + 0x2f0))(iStack_4,10,param_2);
  }
  return;
}

