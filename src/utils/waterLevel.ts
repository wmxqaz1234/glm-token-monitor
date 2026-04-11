/**
 * 根据用量百分比计算水位填充比例
 * 0% 用量 = 100% 填充（满），100% 用量 = 0% 填充（空）
 */
export function calculateFillRatio(usagePercent: number): number {
  return Math.max(0, Math.min(100, 100 - usagePercent)) / 100
}

/**
 * 计算水位线在 viewBox 中的 Y 坐标
 * @param usagePercent 用量百分比 (0-100)
 * @param viewBoxHeight viewBox 高度
 * @param viewBoxBottom viewBox 底部 Y 坐标
 */
export function calculateWaterY(
  usagePercent: number,
  viewBoxHeight: number,
  viewBoxBottom: number
): number {
  const fillRatio = calculateFillRatio(usagePercent)
  return viewBoxBottom - (viewBoxHeight * fillRatio)
}

/**
 * 计算椭圆与水平线的交点
 * @param waterY 水位线 Y 坐标
 * @param centerX 椭圆中心 X
 * @param centerY 椭圆中心 Y
 * @param radiusX 椭圆水平半径
 * @param radiusY 椭圆垂直半径
 */
export function calculateEllipseIntersection(
  waterY: number,
  centerX: number,
  centerY: number,
  radiusX: number,
  radiusY: number
): { left: number; right: number } | null {
  const dy = waterY - centerY
  const ratioSquared = (dy * dy) / (radiusY * radiusY)

  // 如果水位线在椭圆外，无交点
  if (ratioSquared > 1) {
    return null
  }

  const dx = radiusX * Math.sqrt(1 - ratioSquared)
  return {
    left: centerX - dx,
    right: centerX + dx
  }
}

export interface WaterPathResult {
  waterPath: string    // 有水区域的 SVG 路径
  emptyPath: string    // 无水区域的 SVG 路径
  isEmpty: boolean     // 是否完全空
  isFull: boolean      // 是否完全满
}
