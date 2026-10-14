
void __fastcall FUN_0060fb80(int *param_1)

{
  int iVar1;
  int yTop;
  int iVar2;
  int yBottom;
  int iVar3;
  LONG *unaff_retaddr;
  int iVar4;
  int aiStack_20 [2];
  int iStack_18;
  tagRECT tStack_14;
  
  (**(code **)(*param_1 + 0x30))(aiStack_20);
  iVar2 = iStack_18 - aiStack_20[0];
  iVar4 = 0;
  if ((int *)param_1[0x2f] != (int *)0x0) {
    iVar4 = FUN_005fc0e0((int *)param_1[0x2f]);
  }
  iVar1 = param_1[0x37];
  iVar3 = (param_1[0x38] - param_1[0x39]) + iVar1;
  if (iVar1 < 1) {
    iVar1 = 1;
  }
  iVar1 = ((param_1[0x39] - param_1[0x38]) * iVar2) / iVar1;
  if (iVar1 < 0xd) {
    iVar1 = 0xc;
  }
  yTop = aiStack_20[0];
  yBottom = iStack_18;
  if (0 < iVar3) {
    yTop = ((iVar2 - iVar1) * param_1[0x38]) / iVar3 + aiStack_20[0];
    yBottom = yTop + iVar1;
  }
  if (yTop < aiStack_20[0]) {
    yBottom = yBottom + (yTop - aiStack_20[0]);
    yTop = aiStack_20[0];
  }
  if (iStack_18 < yBottom) {
    yTop = yTop + (iStack_18 - yBottom);
    yBottom = iStack_18;
  }
  if (yTop < aiStack_20[0]) {
    yTop = aiStack_20[0];
  }
  iVar2 = (uint)(*(ushort *)(param_1 + 0x2b) >> 1) - iVar4 / 2;
  SetRect(&tStack_14,iVar2,yTop,iVar2 + iVar4,yBottom);
  *unaff_retaddr = tStack_14.left;
  unaff_retaddr[1] = tStack_14.top;
  unaff_retaddr[2] = tStack_14.right;
  unaff_retaddr[3] = tStack_14.bottom;
  return;
}

