
void __fastcall FUN_005f5b20(void *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  
  puVar1 = *(undefined4 **)((int)param_1 + 8);
  while (puVar2 = puVar1, puVar2 != (undefined4 *)0x0) {
    puVar1 = (undefined4 *)puVar2[2];
    FUN_005f5ac0(param_1,(int)puVar2);
    if (puVar2 != (undefined4 *)0x0) {
      (**(code **)*puVar2)(1);
    }
  }
  *(undefined4 *)((int)param_1 + 4) = 0;
  return;
}

