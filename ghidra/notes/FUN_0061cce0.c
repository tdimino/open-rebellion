
void __cdecl
FUN_0061cce0(PEXCEPTION_RECORD param_1,PVOID param_2,DWORD param_3,undefined4 param_4,int param_5,
            int param_6,int param_7,PVOID param_8)

{
  DWORD *pDVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  uint local_8;
  uint local_4;
  
  pDVar1 = FUN_0061d430();
  if ((pDVar1[0x1a] != 0) &&
     (iVar2 = FUN_00618890(&param_1->ExceptionCode,param_2,param_3,param_4,param_5,param_7,param_8),
     iVar2 != 0)) {
    return;
  }
  piVar3 = (int *)FUN_006189f0(param_5,param_7,param_6,&local_8,&local_4);
  if (local_8 < local_4) {
    do {
      if ((*piVar3 <= param_6) && (param_6 <= piVar3[1])) {
        iVar4 = piVar3[4] + piVar3[3] * 0x10;
        iVar2 = *(int *)(iVar4 + -0xc);
        if ((iVar2 == 0) || (*(char *)(iVar2 + 8) == '\0')) {
          FUN_0061ce80(param_1,param_2,param_3,param_4,param_5,(byte *)(iVar4 + -0x10),(byte *)0x0,
                       piVar3,param_7,param_8);
        }
      }
      local_8 = local_8 + 1;
      piVar3 = piVar3 + 5;
    } while (local_8 < local_4);
  }
  return;
}

