
undefined4 __thiscall FUN_005a93e0(void *this,void *param_1)

{
  FUN_005f5440((void *)((int)this + 0xe50),param_1);
  if (*(int *)((int)param_1 + 0x54) != 0) {
    *(undefined4 *)((int)this + *(int *)((int)param_1 + 0x44) * 4 + 0x958) =
         *(undefined4 *)((int)param_1 + 0x18);
    return *(undefined4 *)((int)param_1 + 0x18);
  }
  if (*(int *)((int)param_1 + 0x58) != 0) {
    *(undefined4 *)((int)this + *(int *)((int)param_1 + 0x44) * 4 + 0x950) =
         *(undefined4 *)((int)param_1 + 0x18);
  }
  return *(undefined4 *)((int)param_1 + 0x18);
}

