
void __fastcall FUN_006094b0(void *param_1)

{
  bool bVar1;
  int *piVar2;
  
  bVar1 = false;
  for (piVar2 = (int *)(**(code **)(**(int **)((int)param_1 + 0xa0) + 8))(); piVar2 != (int *)0x0;
      piVar2 = (int *)(**(code **)(*piVar2 + 0xc))()) {
    if ((piVar2[0xf] & 1U) != 0) {
      bVar1 = true;
    }
    piVar2[0xf] = piVar2[0xf] & 0xfffffffe;
  }
  if (bVar1) {
    FUN_00609a00(param_1);
  }
  return;
}

