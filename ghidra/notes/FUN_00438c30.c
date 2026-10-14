
void __fastcall FUN_00438c30(void *param_1)

{
  FUN_00438d30((int)param_1);
  if (*(int *)((int)param_1 + 0x148) < 0xff) {
    *(int *)((int)param_1 + 0x148) = *(int *)((int)param_1 + 0x148) + 1;
    FUN_00438c90((int)param_1);
    FUN_00439160(param_1);
  }
  return;
}

