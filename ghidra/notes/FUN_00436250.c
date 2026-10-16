
uint __fastcall FUN_00436250(int param_1)

{
  uint uVar1;
  uint uVar2;
  
  uVar1 = (**(code **)(**(int **)(param_1 + 0x10) + 0x2c))();
  uVar2 = (**(code **)(**(int **)(param_1 + 0x14) + 0x2c))();
  return uVar1 & 1 & uVar2;
}

