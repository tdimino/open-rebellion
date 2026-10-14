
uint FUN_0061a310(void)

{
  uint uVar1;
  DWORD *pDVar2;
  
  pDVar2 = FUN_0061d430();
  uVar1 = pDVar2[5] * 0x343fd + 0x269ec3;
  pDVar2[5] = uVar1;
  return uVar1 >> 0x10 & 0x7fff;
}

