
undefined4 __thiscall
FUN_00485180(void *this,undefined4 *param_1,undefined4 *param_2,undefined4 *param_3)

{
  undefined4 *puVar1;
  
  if (*(int *)((int)this + 0x38) != 0) {
    *param_1 = *(undefined4 *)((int)this + 0x44);
    FUN_004f26d0(param_2,(undefined4 *)((int)this + 0x40));
    FUN_004fcd90(param_3,(undefined4 *)((int)this + 0x3c));
    return *(undefined4 *)((int)this + 0x38);
  }
  puVar1 = (undefined4 *)
           FUN_004be780((void *)((int)this + 0x58),(int *)((int)this + 0x40),
                        *(uint *)((int)this + 0x44),1);
  if (puVar1 != (undefined4 *)0x0) {
    *(undefined4 *)((int)this + 0x38) = puVar1[7];
    FUN_004fcd90((undefined4 *)((int)this + 0x3c),puVar1 + 9);
    FUN_004fcd90(param_3,(undefined4 *)((int)this + 0x3c));
    *param_1 = puVar1[6];
    FUN_004f26d0(param_2,puVar1 + 10);
    FUN_005f4fa0((void *)((int)this + 0x58),(int)puVar1);
    (**(code **)*puVar1)(1);
    return *(undefined4 *)((int)this + 0x38);
  }
  *param_1 = 0;
  FUN_004ece80(param_2);
  FUN_004fcdd0(param_3);
  return *(undefined4 *)((int)this + 0x38);
}

