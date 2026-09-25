
void __fastcall FUN_00416de0(void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = FUN_006069d0(param_1,0);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    if (*(UINT *)((int)param_1 + 0x118) != 0) {
      FUN_00606b40(param_1,*(UINT *)((int)param_1 + 0x118));
    }
    if (*(UINT *)((int)param_1 + 0x11c) != 0) {
      FUN_00606b40(param_1,*(UINT *)((int)param_1 + 0x11c));
    }
    FUN_00606ab0(param_1);
  }
  return;
}

