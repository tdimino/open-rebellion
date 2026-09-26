
void __fastcall FUN_004bf7c0(int param_1)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a10d;
  local_c = ExceptionList;
  switch(*(undefined4 *)(param_1 + 0x5c)) {
  case 1:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x30);
    local_4 = 6;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004e60e0(pvVar1,*(undefined4 *)(param_1 + 0x30),*(undefined4 *)(param_1 + 0x2c));
      *(void **)(param_1 + 0x60) = pvVar1;
      ExceptionList = local_c;
      return;
    }
    break;
  case 2:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x30);
    local_4 = 4;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004e77c0(pvVar1,*(undefined4 *)(param_1 + 0x30),*(undefined4 *)(param_1 + 0x2c));
      *(void **)(param_1 + 0x60) = pvVar1;
      ExceptionList = local_c;
      return;
    }
    break;
  case 3:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x30);
    local_4 = 3;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004e8770(pvVar1,*(undefined4 *)(param_1 + 0x30),*(undefined4 *)(param_1 + 0x2c));
      *(void **)(param_1 + 0x60) = pvVar1;
      ExceptionList = local_c;
      return;
    }
    break;
  case 4:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x30);
    local_4 = 2;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004e94a0(pvVar1,*(undefined4 *)(param_1 + 0x30),*(undefined4 *)(param_1 + 0x2c));
      *(void **)(param_1 + 0x60) = pvVar1;
      ExceptionList = local_c;
      return;
    }
    break;
  case 5:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x30);
    local_4 = 1;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004e9e90(pvVar1,*(undefined4 *)(param_1 + 0x30),*(undefined4 *)(param_1 + 0x2c));
      *(void **)(param_1 + 0x60) = pvVar1;
      ExceptionList = local_c;
      return;
    }
    break;
  case 6:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x30);
    local_4 = 0;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004eab80(pvVar1,*(undefined4 *)(param_1 + 0x30),*(undefined4 *)(param_1 + 0x2c));
      *(void **)(param_1 + 0x60) = pvVar1;
      ExceptionList = local_c;
      return;
    }
    break;
  case 7:
    ExceptionList = &local_c;
    pvVar1 = (void *)FUN_00618b70(0x30);
    local_4 = 5;
    if (pvVar1 != (void *)0x0) {
      pvVar1 = FUN_004e6a30(pvVar1,*(undefined4 *)(param_1 + 0x30),*(undefined4 *)(param_1 + 0x2c));
      *(void **)(param_1 + 0x60) = pvVar1;
      ExceptionList = local_c;
      return;
    }
    break;
  default:
    goto switchD_004bf7e6_default;
  }
  *(undefined4 *)(param_1 + 0x60) = 0;
switchD_004bf7e6_default:
  ExceptionList = local_c;
  return;
}

