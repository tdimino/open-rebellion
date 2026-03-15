
void __fastcall FUN_005cb6e0(int param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  undefined3 extraout_var;
  void *this;
  int iVar4;
  float in_stack_ffffffdc;
  float in_stack_ffffffe0;
  float in_stack_ffffffe4;
  undefined4 in_stack_ffffffe8;
  
  (**(code **)(*(int *)(param_1 + 0xa4) + 4))();
  iVar2 = (**(code **)(*(int *)(param_1 + 0xa4) + 0x10))();
  if (iVar2 != 0) {
    return;
  }
  iVar4 = 0;
  iVar2 = FUN_005c6070((int *)(param_1 + 0x1c));
  if (iVar2 == 0) {
    return;
  }
  iVar3 = FUN_005b7ce0(iVar2);
  if (iVar3 == 0) {
    iVar2 = FUN_005b7d60(iVar2);
    if (iVar2 == 0) goto LAB_005cb754;
    iVar2 = 0;
  }
  else {
    iVar2 = 1;
  }
  FUN_005a2260((void *)(*(int *)(param_1 + 0x54) + 0xc),(undefined4 *)&stack0xffffffdc);
  bVar1 = FUN_005cbce0(param_1);
  iVar4 = CONCAT31(extraout_var,bVar1);
  this = (void *)FUN_005aaf90();
  iVar4 = FUN_005aa970(this,iVar4,in_stack_ffffffdc,in_stack_ffffffe0,in_stack_ffffffe4,
                       in_stack_ffffffe8,iVar2);
LAB_005cb754:
  if (iVar4 != 0) {
    FUN_005d2080((int *)(param_1 + 0xa4),iVar4);
  }
  return;
}

