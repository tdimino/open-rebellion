
void __fastcall FUN_005d6bb0(int param_1)

{
  int *piVar1;
  
  DAT_006bcbb4 = DirectDrawCreateClipper(0,(undefined4 *)(param_1 + 0xec),0);
  if (DAT_006bcbb4 != 0) {
    FUN_005d93f0(0,DAT_006bcbb4);
  }
  piVar1 = *(int **)(param_1 + 0xec);
  DAT_006bcbb4 = (**(code **)(*piVar1 + 0x20))(piVar1,0,*(undefined4 *)(param_1 + 0x18));
  if (DAT_006bcbb4 != 0) {
    FUN_005d93f0(0,DAT_006bcbb4);
  }
  return;
}

