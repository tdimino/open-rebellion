
int __fastcall FUN_0053e210(int param_1)

{
  int iVar1;
  int iVar2;
  int unaff_EBX;
  int iVar3;
  int unaff_EBP;
  int unaff_ESI;
  int unaff_EDI;
  undefined1 in_ZF;
  char in_SF;
  char in_OF;
  
  while( true ) {
    iVar2 = param_1;
    iVar3 = iVar2;
    if (((bool)in_ZF || in_OF != in_SF) && (iVar3 = unaff_EBX, in_OF != in_SF)) {
      unaff_EDI = iVar2;
    }
    if (iVar2 == unaff_EBP) break;
    param_1 = (unaff_EDI + iVar3) / 2;
    in_OF = SBORROW4(param_1 * param_1,unaff_ESI);
    iVar1 = param_1 * param_1 - unaff_ESI;
    in_SF = iVar1 < 0;
    in_ZF = iVar1 == 0;
    unaff_EBX = iVar3;
    unaff_EBP = iVar2;
  }
  return iVar2;
}

