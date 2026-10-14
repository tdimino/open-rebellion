
void __fastcall FUN_00486520(int param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined4 *puVar2;
  undefined3 extraout_var_00;
  
  if (*(int *)(param_1 + 0xc0) != 0) {
    bVar1 = FUN_00487f30(param_1);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      FUN_00439bb0(*(int **)(param_1 + 0xc0));
    }
  }
  puVar2 = (undefined4 *)FUN_004befe0((void *)(param_1 + 0x5c));
  if (puVar2 != (undefined4 *)0x0) {
    FUN_00488b50((void *)(param_1 + 0x6c),(int)puVar2);
    (**(code **)*puVar2)(1);
  }
  if (*(int *)(param_1 + 0x48) == 0) {
    bVar1 = FUN_0041dc10();
    if (CONCAT31(extraout_var_00,bVar1) == 0) {
      FUN_00488b90(param_1 + 0x6c);
    }
  }
  return;
}

