
void __fastcall FUN_005c7390(undefined4 *param_1)

{
  *param_1 = &PTR_FUN_0066be08;
  param_1[7] = &PTR_LAB_0066be00;
  param_1[10] = &PTR_LAB_0066bdfc;
  param_1[0xc] = &PTR_LAB_0066bdf0;
  param_1[0xe] = &PTR_LAB_0066bdec;
  *(undefined ***)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0xe)) =
       &PTR_LAB_0066bde8;
  *(undefined ***)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0xc)) =
       &PTR_LAB_0066be18;
  *(undefined ***)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 10)) = &PTR_LAB_0066be24
  ;
  *(undefined ***)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 7)) = &PTR_LAB_0066be28;
  FUN_005a6280(param_1);
  return;
}

