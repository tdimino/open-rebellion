
void __cdecl
FUN_0061f540(uint *param_1,uint *param_2,byte param_3,uint param_4,uint *param_5,uint *param_6)

{
  uint *puVar1;
  uint *puVar2;
  uint uVar3;
  uint *dwExceptionCode;
  
  puVar1 = param_2;
  param_1[1] = 0;
  param_1[2] = 0;
  param_1[3] = 0;
  dwExceptionCode = param_1;
  if ((param_3 & 0x10) != 0) {
    param_1[1] = param_1[1] | 1;
    dwExceptionCode = (uint *)0xc000008f;
  }
  if ((param_3 & 2) != 0) {
    dwExceptionCode = (uint *)0xc0000093;
    param_1[1] = param_1[1] | 2;
  }
  if ((param_3 & 1) != 0) {
    dwExceptionCode = (uint *)0xc0000091;
    param_1[1] = param_1[1] | 4;
  }
  if ((param_3 & 4) != 0) {
    dwExceptionCode = (uint *)0xc000008e;
    param_1[1] = param_1[1] | 8;
  }
  if ((param_3 & 8) != 0) {
    dwExceptionCode = (uint *)0xc0000090;
    param_1[1] = param_1[1] | 0x10;
  }
  param_1[2] = (~*param_2 & 1) << 4 | param_1[2] & 0xffffffef;
  param_1[2] = (~*param_2 & 4) << 1 | param_1[2] & 0xfffffff7;
  param_1[2] = ~*param_2 >> 1 & 4 | param_1[2] & 0xfffffffb;
  param_1[2] = ~*param_2 >> 3 & 2 | param_1[2] & 0xfffffffd;
  param_1[2] = ~*param_2 >> 5 & 1 | param_1[2] & 0xfffffffe;
  uVar3 = FUN_0061feb0();
  puVar2 = param_6;
  if ((uVar3 & 1) != 0) {
    param_1[3] = param_1[3] | 0x10;
  }
  if ((uVar3 & 4) != 0) {
    param_1[3] = param_1[3] | 8;
  }
  if ((uVar3 & 8) != 0) {
    param_1[3] = param_1[3] | 4;
  }
  if ((uVar3 & 0x10) != 0) {
    param_1[3] = param_1[3] | 2;
  }
  if ((uVar3 & 0x20) != 0) {
    param_1[3] = param_1[3] | 1;
  }
  uVar3 = *puVar1 & 0xc00;
  if (uVar3 < 0x401) {
    if (uVar3 == 0x400) {
      *param_1 = *param_1 & 0xfffffffd | 1;
    }
    else if (uVar3 == 0) {
      *param_1 = *param_1 & 0xfffffffc;
    }
  }
  else if (uVar3 == 0x800) {
    *param_1 = *param_1 & 0xfffffffe | 2;
  }
  else if (uVar3 == 0xc00) {
    *param_1 = *param_1 | 3;
  }
  uVar3 = *puVar1 & 0x300;
  if (uVar3 == 0) {
    *param_1 = *param_1 & 0xffffffeb | 8;
  }
  else if (uVar3 == 0x200) {
    *param_1 = *param_1 & 0xffffffe7 | 4;
  }
  else if (uVar3 == 0x300) {
    *param_1 = *param_1 & 0xffffffe3;
  }
  *param_1 = *param_1 & 0xfffe001f | (param_4 & 0xfff) << 5;
  param_1[8] = param_1[8] | 1;
  param_1[8] = param_1[8] & 0xffffffe3 | 2;
  param_1[4] = *param_5;
  param_1[5] = param_5[1];
  param_1[0x14] = param_1[0x14] | 1;
  param_1[0x14] = param_1[0x14] & 0xffffffe3 | 2;
  param_1[0x10] = *param_6;
  param_1[0x11] = param_6[1];
  FUN_0061fec0();
  RaiseException((DWORD)dwExceptionCode,0,1,(ULONG_PTR *)&param_1);
  if ((param_1[2] & 0x10) != 0) {
    *puVar1 = *puVar1 & 0xfffffffe;
  }
  if ((param_1[2] & 8) != 0) {
    *puVar1 = *puVar1 & 0xfffffffb;
  }
  if ((param_1[2] & 4) != 0) {
    *puVar1 = *puVar1 & 0xfffffff7;
  }
  if ((param_1[2] & 2) != 0) {
    *puVar1 = *puVar1 & 0xffffffef;
  }
  if ((param_1[2] & 1) != 0) {
    *puVar1 = *puVar1 & 0xffffffdf;
  }
  switch(*param_1 & 3) {
  case 0:
    uVar3 = *puVar1 & 0xfffff3ff;
    break;
  case 1:
    *puVar1 = *puVar1 & 0xfffff7ff | 0x400;
    goto switchD_0061f7db_default;
  case 2:
    uVar3 = *puVar1 & 0xfffffbff | 0x800;
    break;
  case 3:
    uVar3 = *puVar1 | 0xc00;
    break;
  default:
    goto switchD_0061f7db_default;
  }
  *puVar1 = uVar3;
switchD_0061f7db_default:
  uVar3 = *param_1 >> 2 & 7;
  if (uVar3 == 0) {
    *puVar1 = *puVar1 & 0xfffff3ff | 0x300;
  }
  else {
    if (uVar3 == 1) {
      *puVar1 = *puVar1 & 0xfffff3ff | 0x200;
      *puVar2 = param_1[0x10];
      puVar2[1] = param_1[0x11];
      return;
    }
    if (uVar3 == 2) {
      *puVar1 = *puVar1 & 0xfffff3ff;
      *puVar2 = param_1[0x10];
      puVar2[1] = param_1[0x11];
      return;
    }
  }
  *puVar2 = param_1[0x10];
  puVar2[1] = param_1[0x11];
  return;
}

